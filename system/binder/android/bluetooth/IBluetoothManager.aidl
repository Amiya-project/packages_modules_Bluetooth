/*
 * Copyright 2012 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package android.bluetooth;

import android.bluetooth.IBluetooth;
import android.bluetooth.IBluetoothGatt;
import android.bluetooth.IBluetoothManagerCallback;
import android.bluetooth.IBluetoothProfileServiceConnection;
import android.bluetooth.IBluetoothStateChangeCallback;

/**
 * System private API for talking with the Bluetooth service.
 *
 * {@hide}
 */
interface IBluetoothManager
{
    IBluetooth registerAdapter(in IBluetoothManagerCallback callback);
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    void unregisterAdapter(in IBluetoothManagerCallback callback);
    @UnsupportedAppUsage
    void registerStateChangeCallback(in IBluetoothStateChangeCallback callback);
    @UnsupportedAppUsage
    void unregisterStateChangeCallback(in IBluetoothStateChangeCallback callback);
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    boolean enable(String packageName);
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    boolean enableNoAutoConnect(String packageName);
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    boolean disable(String packageName, boolean persist);
    @JavaPassthrough(annotation="@android.annotation.RequiresNoPermission")
    int getState();
    @UnsupportedAppUsage
    @JavaPassthrough(annotation="@android.annotation.RequiresNoPermission")
    IBluetoothGatt getBluetoothGatt();

    boolean bindBluetoothProfileService(int profile, IBluetoothProfileServiceConnection proxy);
    void unbindBluetoothProfileService(int profile, IBluetoothProfileServiceConnection proxy);

    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(allOf={android.Manifest.permission.BLUETOOTH_CONNECT,android.Manifest.permission.LOCAL_MAC_ADDRESS})")
    String getAddress();
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    String getName();

    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_PRIVILEGED)")
    boolean onFactoryReset();

    @JavaPassthrough(annotation="@android.annotation.RequiresNoPermission")
    boolean isBleScanAlwaysAvailable();
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    boolean enableBle(String packageName, IBinder b);
    @JavaPassthrough(annotation="@android.annotation.RequiresPermission(android.Manifest.permission.BLUETOOTH_CONNECT)")
    boolean disableBle(String packageName, IBinder b);
    @JavaPassthrough(annotation="@android.annotation.RequiresNoPermission")
    boolean isBleAppPresent();
    @JavaPassthrough(annotation="@android.annotation.RequiresNoPermission")
    boolean isHearingAidProfileSupported();

    @JavaPassthrough(annotation="@android.annotation.RequiresNoPermission")
    List<String> getSystemConfigEnabledProfilesForPackage(String packageName);
}
